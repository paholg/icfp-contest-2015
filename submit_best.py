#!/usr/bin/env python2
import subprocess
import shlex
import re

API_TOKEN = 'FtpwGAy9ndcLXLUlH7i96rgXLgi2SzEdym2caXEsNUI='
TEAM_ID = '97'

cmd = shlex.split("ls solutions")
out_files = subprocess.Popen(cmd, stdout=subprocess.PIPE).stdout.readlines()

files = []
my_list = []
out_json_files = []

for s in out_files:
    match_obj = re.match( r'(.*)\-(.*)\-(.*)\.json', s)
    files.append( (match_obj.group(1), match_obj.group(2), match_obj.group(3)) )

for n in range(0,25):
    my_list = []
    for i in files:
        if int(i[0]) == n:
            my_list.append(i)
    #print sorted(my_list, key=lambda item: item[2], reverse=True)[0]
    out_json_files.append( sorted(my_list, key=lambda item: item[2], reverse=True)[0] )

for e in out_json_files:
    fname = e[0] + "-" + e[1] + "-" + e[2] + ".json"
    post_cmd = "curl --user :" + str(API_TOKEN) + " -X POST -H \"Content-Type: application/json " + \
            "-d " + fname + " https://davar.icfpcontest.org/teams/" + str(TEAM_ID) + "/solutions"
    print post_cmd