#!/bin/sh

cd /home/minnehack/Opaque/storage &&
    git add * &&
    git commit -m 'update resumes' &&
    git push -u <REPO HERE> <BRANCH HERE>
