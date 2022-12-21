# ws-checker

Check website availability from a domain or a list of domains.

The program will determine if a domain's website is up or down visiting the following URLs in order:

- https://www.domain.com
- https://domain.com
- http://www.domain.com
- http://domain.com

Once a website URL is found to be up, the program stops and returns the URL.

Check one domain
```bash
ws-checker example.com
```

To check multiple domains from a CSV file, the csv file name should be domains.csv and it should be in the same directory as the executable file.

```bash
ws-checker
```

Results are saved into the `reports.csv` file
