import requests
while True:
    inp = input("pkgname: ")
    res = requests.get(f'http://registry-19d90.kxcdn.com/{inp}.json')
    print('KeyCDN: ', res.elapsed.total_seconds())

    res = requests.get(f'http://volt-api.b-cdn.net/{inp}.json')
    print('BunnyCDN: ', res.elapsed.total_seconds())

    res = requests.get(f'http://push-2105.5centscdn.com/{inp}.json')
    print('5CentsCDN: ', res.elapsed.total_seconds())