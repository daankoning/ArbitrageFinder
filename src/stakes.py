"""Named-outcome, budget-constrained settlement analysis; no execution."""
import math


def allocate(outcomes, budget=100, fee=0):
    if not math.isfinite(budget) or budget < .01 or budget > 1e7:
        raise ValueError('Budget must be finite and between 0.01 and 10,000,000')
    if not math.isfinite(fee) or not 0 <= fee < 1:
        raise ValueError('Profit commission must be between 0 and 1')
    if len(outcomes) < 2:
        raise ValueError('At least two exhaustive outcomes are required')
    legs = []
    for name, (bookmaker, odds) in outcomes.items():
        if not name or not math.isfinite(float(odds)) or float(odds) <= 1:
            raise ValueError('Each named outcome needs finite decimal odds greater than one')
        effective = 1 + (float(odds) - 1) * (1 - fee)
        legs.append({'outcome': name, 'bookmaker': bookmaker, 'odds': float(odds), 'effectiveOdds': effective})
    inverse = sum(1 / leg['effectiveOdds'] for leg in legs)
    cents = math.floor(budget * 100 + 1e-7)
    ideal = [cents / leg['effectiveOdds'] / inverse for leg in legs]
    units = [math.floor(value) for value in ideal]
    # Largest-remainder apportionment spends exactly the available whole cents.
    order = sorted(range(len(legs)), key=lambda i: (-(ideal[i] - units[i]), i))
    for i in order[:cents - sum(units)]:
        units[i] += 1
    spent = sum(units) / 100
    for leg, value in zip(legs, units):
        leg['stake'] = value / 100
        leg['netIfWins'] = leg['stake'] * leg['effectiveOdds'] - spent
    worst = min(leg['netIfWins'] for leg in legs)
    return {'budget': budget, 'spent': spent, 'unspent': budget - spent, 'impliedSum': inverse,
            'theoreticalReturn': 1 / inverse - 1, 'worstNet': worst, 'worstReturn': worst / spent,
            'legs': legs, 'allLegsFunded': all(leg['stake'] > 0 for leg in legs)}
