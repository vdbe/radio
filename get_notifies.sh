#!/usr/bin/env sh

SCRIPT_PATH=$(dirname "$0")

. "${SCRIPT_PATH}/.env"

test -z $1 || HOST=$1
test -z $2 || PIN=$2

sid=$(curl -sS "http://${HOST}/fsapi/CREATE_SESSION?pin=${PIN}" | \
	grep 'sessionId' | \
	sed --expression='s/^<sessionId>\(.*\)<\/sessionId>$/\1/')

echo "Press <CTRL+C> to exit."
while :
do
	curl -sS "http://${HOST}/fsapi/GET_NOTIFIES?pin=${PIN}&sid=${sid}" | \
		grep "<notify" | \
		sed --expression='s/^<notify node="\(.*\)"><value>\(.*\)<\/value>$/\1: \2/'
done
