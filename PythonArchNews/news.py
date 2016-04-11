import urllib
import re
import feedparser

archFeed = "https://www.archlinux.org/feeds/packages/x86_64/"
bbcFeed = "http://feeds.bbci.co.uk/news/world/rss.xml#"

output = ""	
#result = re.search('>.*<',fileString) #pull out our "nothing" number
#if result is not None: 
	# output += result.group(0)+"\n" #set our nothing to the value from the regex

def parseRss(feed) :	
	output = feedparser.parse(feed)
	print output.feed.title
	print output.feed.description+"\n"
	
	for entry in output.entries :
		print entry.published
		print entry.title+"\n"
		print entry.description+"\n"
		
		#for descLine in entry.description :
		#	 print "\t"+descLine
		

parseRss(bbcFeed)
