import requests
from bs4 import BeautifulSoup
import re

password = "natas0"
username = "natas0"
url = f"http://{username}.natas.labs.overthewire.org"

""" Pulling data from the website"""
data = requests.post(url, auth=(username, password))
soup = BeautifulSoup(data.content, "html.parser")

result = re.findall(r'[A-Za-z0-9]{32}', data.text)[0]
print(result)
