from typing import Iterable, Generator
import time
import requests

BASE_URL = "https://api.the-odds-api.com/v4"


def get_sports(key: str) -> set[str]:
	url = f"{BASE_URL}/sports/"
	querystring = {"apiKey": key}

	response = requests.get(url, params=querystring)
	return {item["group"] for item in response.json()}


def get_data(key: str, sport: str, region: str = "eu"):
	url = f"{BASE_URL}/sports/{sport}/odds/"
	querystring = {
		"apiKey": key,
		"regions": region,
		"oddsFormat": "decimal",
		"dateFormat": "unix"
	}

	response = requests.get(url, params=querystring)
	return response.json()


def process_data(matches: Iterable, include_started_matches: bool = True) -> Generator[dict, None, None]:
	"""Extracts all matches that are available and calculates some things about them, such as the time to start and
	the best available implied odds."""
	for match in matches:
		start_time = int(match["commence_time"])
		if not include_started_matches and start_time < time.time():
			continue

		best_odd_per_outcome = {}
		for bookmaker in match["bookmakers"]:
			bookie_name = bookmaker["title"]
			for outcome in bookmaker["markets"][0]["outcomes"]:
				outcome_name = outcome["name"]
				odd = outcome["price"]
				if outcome_name not in best_odd_per_outcome.keys() or \
					odd > best_odd_per_outcome[outcome_name][1]:
					best_odd_per_outcome[outcome_name] = (bookie_name, odd)

		total_implied_odds = sum(1/i[1] for i in best_odd_per_outcome.values())
		match_name = f"{match['home_team']} v. {match['away_team']}"
		time_to_start = (start_time - time.time())/3600
		league = match["sport_key"]
		yield {
			"match_name": match_name,
			"match_start_time": start_time,
			"time_to_start": time_to_start,
			"league": league,
			"best_outcome_odds": best_odd_per_outcome,
			"total_implied_odds": total_implied_odds,
		}