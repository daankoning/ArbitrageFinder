from logic import *
import os
from itertools import chain
import argparse
from dotenv import load_dotenv
try:
	from rich import print
except ImportError:
	pass


def main():
	load_dotenv()

	parser = argparse.ArgumentParser(
		prog="Arbitrage Finder",
		description="""A simple tool to find sports betting arbitrage opportunities.
		
		The tool fetches the odds from The Odds API (https://the-odds-api.com/) and compares the odds at different
		bookmakers to each other in order to determine whether there are profitable and risk-free bets available."""
	)
	parser.add_argument(
		"-k", "--key",
		default=os.environ.get("API_KEY"),
		help="The API key from The Odds API. If left blank it will default to the value of $API_KEY."
	)
	parser.add_argument(
		"-r", "--region",
		choices=["eu", "us", "au", "uk"],
		default="eu",
		help="The region in which to look for arbitrage opportunities."
	)
	args = parser.parse_args()

	key = args.key
	region = args.region

	# logic
	sports = get_sports(key)
	data = chain.from_iterable(get_data(key, sport, region=region) for sport in sports)
	data = filter(lambda x: x != "message", data)
	results = process_data(data)

	arbitrage_opportunities = filter(lambda x: x["total_implied_odds"] < 1, results)
	for i in arbitrage_opportunities:
		print(i)


if __name__ == '__main__':
	main()