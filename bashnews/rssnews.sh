#! /bin/bash
#RSS news bash script by Rave Kutsuu

while ! iwgetid -r 
do
 echo "Waiting for wireless network.. (Retry in 15 minutes)"
 sleep 15m
done

echo "passed"
wget -q -O- "http://feeds.bbci.co.uk/news/rss.xml?edition=int" | egrep -o -i '<title.*|<description.*' --max-count=9 | sed -e "s/<\/[^>]*>/\n/g"  -e "s/<[^>]*>//g"
