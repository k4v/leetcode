def maxProfit(self, prices: List[int]) -> int:
    sell_price = buy_price = prices[0]

        total_profit = 0

        for price in prices:
            if price < buy_price:
                total_profit = max(total_profit, sell_price - buy_price)
                buy_price = sell_price = price
            if price > sell_price:
                sell_price = price
        return max(total_profit, sell_price - buy_price)