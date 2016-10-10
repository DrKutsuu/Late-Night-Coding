import urllib
import re
import feedparser

archFeed = "https://www.archlinux.org/feeds/packages/x86_64/?action=rss_rc"
bbcFeed = "http://feeds.bbci.co.uk/news/world/rss.xml?action=rss_rc"

def parseRss(feedUrl) :	
	feed = feedparser.parse(feedUrl)
	#print (feed.title)
	#print (feed.description+"\n")
	
	print (feed["channel"]["title"])
	print (feed["channel"]["link"]+"\n")
	
	for entry in feed["items"] :
		#print (entry.published)
		print (entry["title"])
		print (entry["description"]+"\n")
	
	print("------------------")	
		

parseRss(archFeed)
parseRss(bbcFeed)
