from logic import *
import os
from itertools import chain
from dotenv import load_dotenv
from rich import print


load_dotenv()


def main():
	key = os.environ.get("API_KEY")

	sports = get_sports(key)
	data = chain.from_iterable(get_data(key, sport) for sport in sports)
	data = filter(lambda x: x != "message", data)
	results = process_data(data)

	arbitrage_opportunities = filter(lambda x: x["total_implied_odds"] < 1, results)
	for i in arbitrage_opportunities:
		print(i)


if __name__ == '__main__':
	main()