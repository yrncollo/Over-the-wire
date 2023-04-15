import requests
from bs4 import BeautifulSoup
import re

password = "g9D9cREhslqBKtcA2uocGHPfMZVzeFK6"
username = "natas1"
url = f"http://{username}.natas.labs.overthewire.org"

website = requests.get(url, auth=(username, password))
soup = BeautifulSoup(website.content, "html.parser")

# print(soup.prettify())
result = re.findall(r'[A-Za-z0-9]{32}', website.text)[1]

print(result)
