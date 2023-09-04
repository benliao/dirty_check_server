
import requests
from urllib.parse import quote

base_url = "http://localhost:8333/api/dirtycheck/"
content = "强奸"
url = base_url + quote(content, safe='')

r = requests.get(url)
print(r.json())

