<p style="font-size: 5em; text-align: center;">Coin Counter</p>
<p style="font-size: 2em; text-align: center;">Evan Drake</p>

[TOC]

# Problem statement
we will be taking in some number and with it a currency type. we will then find the lowest amount of this currencies coins to fill the amount.
#### example 
we would be given some amount .24 with the coin values being worth 0.3 0.9 0.18 0.24 we would then need to find the lowest amount of coins that produce the amount .24. in this case it is the 0.24 coin. the greedy method though tempting is not necisarily the correct answer. 
### greedy method

given the set coins = {coin1, coin2, coin3, coin4} and the currency cur.
```pseudocode
	\\sort the coins in decending order
	sort(coins)
	for i in range(0, coins.length)
		while coins[0] > cur
			cur -= coins[0]
			coin_num += 1
	return coin_num
```
This is a rather challenging problem and the answer is not cheap and requires research. of course the goal should be to get a working version utilizing the greedy method first. once you have accomplished this then you can start work on a better version if you wish. use paper and start using US currency until that works then draw out some plans not using code that may solve the problem better. I am on discord for help and will be working thourgh an optimized version of the problem myself.