from typing import Iterable, Generator
import time
import math
import requests
from itertools import chain

try:
    from tqdm import tqdm
except ImportError:
    tqdm = lambda *args, **kwargs: args[0]

BASE_URL = "api.the-odds-api.com/v4"
PROTOCOL = "https://"


class APIException(RuntimeError):
    def __str__(self):
        return str(self.args[0]) + (f" (HTTP {self.args[1].status_code})" if len(self.args) > 1 else "")


class AuthenticationException(APIException):
    pass


class RateLimitException(APIException):
    pass


class TimeoutException(APIException):
    pass


def handle_faulty_response(response: requests.Response):
    if response.status_code == 401:
        raise AuthenticationException("Failed to authenticate with the API. is the API key valid?", response)
    elif response.status_code == 429:
        raise RateLimitException("Encountered API rate limit.", response)
    else:
        raise APIException("Unknown issue arose while trying to access the API.", response)


def get_sports(key: str) -> set[str]:
    url = f"{BASE_URL}/sports/"
    escaped_url = PROTOCOL + requests.utils.quote(url)
    querystring = {"apiKey": key}

    try:
        response = requests.get(escaped_url, params=querystring, timeout=30)
    except requests.exceptions.Timeout:
        raise TimeoutException("Timed out while trying to access the API.")
    if not response:
        handle_faulty_response(response)

    return {item["key"] for item in response.json()}


def get_data(key: str, sport: str, region: str = "eu"):
    url = f"{BASE_URL}/sports/{sport}/odds/"
    escaped_url = PROTOCOL + requests.utils.quote(url)
    querystring = {
        "apiKey": key,
        "regions": region,
        "oddsFormat": "decimal",
        "dateFormat": "unix",
        "markets": "h2h"
    }

    try:
        response = requests.get(escaped_url, params=querystring, timeout=30)
    except requests.exceptions.Timeout:
        raise TimeoutException("Timed out while trying to access the API.")
    if not response:
        handle_faulty_response(response)

    return response.json()


def process_data(matches: Iterable, include_started_matches: bool = False) -> Generator[dict, None, None]:
    """Extracts all matches that are available and calculates some things about them, such as the time to start and
    the best available implied odds."""
    matches = tqdm(matches, desc="Checking all matches", leave=False, unit=" matches")
    for match in matches:
        start_time = int(match["commence_time"])
        if not include_started_matches and start_time < time.time():
            continue

        expected = {match['home_team'], match['away_team']}
        if match['sport_key'].startswith('soccer_'):
            expected.add('Draw')
        # A draw reported by any h2h book is part of the exhaustive outcome set.
        markets = [(book, market) for book in match.get('bookmakers', []) for market in book.get('markets', []) if market.get('key') == 'h2h']
        if any(outcome.get('name') == 'Draw' for _, market in markets for outcome in market.get('outcomes', [])):
            expected.add('Draw')
        best_odd_per_outcome = {}
        for bookmaker, market in markets:
            outcomes = market.get('outcomes', [])
            if {o.get('name') for o in outcomes} != expected or len(outcomes) != len(expected):
                continue
            if any(not isinstance(o.get('price'), (int, float)) or not math.isfinite(o['price']) or o['price'] <= 1 for o in outcomes):
                continue
            for outcome in outcomes:
                name, odds = outcome['name'], outcome['price']
                if name not in best_odd_per_outcome or odds > best_odd_per_outcome[name][1]:
                    best_odd_per_outcome[name] = (bookmaker['title'], odds)
        if set(best_odd_per_outcome) != expected:
            continue

        total_implied_odds = sum(1/i[1] for i in best_odd_per_outcome.values())
        match_name = f"{match['home_team']} v. {match['away_team']}"
        time_to_start = (start_time - time.time())/3600
        league = match["sport_key"]
        yield {
            "match_name": match_name,
            "match_start_time": start_time,
            "hours_to_start": time_to_start,
            "league": league,
            "best_outcome_odds": best_odd_per_outcome,
            "total_implied_odds": total_implied_odds,
        }


def get_arbitrage_opportunities(key: str, region: str, cutoff: float):
    sports = get_sports(key)
    data = chain.from_iterable(get_data(key, sport, region=region) for sport in sports)
    data = filter(lambda x: x != "message", data)
    results = process_data(data)
    arbitrage_opportunities = filter(lambda x: 0 < x["total_implied_odds"] < 1 / (1 + cutoff), results)

    return arbitrage_opportunities
