# How to make a fast website

Using htmx - rust - postres - caddy

I heard many times that 100 ms latency for a web app is fast. While this latency may be okay, it is for sure not fast.

| Speed | Latency              |
| ----- | -------------------- |
| Slow  | > ping time + 500 ms |
| Okay  | < ping time + 100 ms |
| Fast  | < ping time + 5 ms   |

Of course this seems impossible?

Visit my [music database](https://taste.silenlocatelli.ch) to find out if is also fast for you.

How did I achieve it:

- database is running inside of the same vps
- the app only replaces the table with the filters nothing else
- the query is simple and limits the return on 10

Also see https://taste.silenlocatelli.ch/about
