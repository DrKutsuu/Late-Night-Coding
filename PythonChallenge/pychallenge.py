#!/bin/python
#An attempt to learn python while completing the challenges specified at http://www.pythonchallenge.com/
import re
import urllib
import pickle 

alphabet = "abcdefghijklmnopqrstuvwxyz"

def transform(inputString,shiftAmount) :
	output = "";
	
	for index in range(len(inputString)) :
		thisChar = inputString[index];
		alphaIndex = alphabet.find(thisChar)
		
		if (alphaIndex == -1) :
			output += thisChar
			continue
		if (alphaIndex >= (len(alphabet)-shiftAmount)) :
			alphaIndex = alphaIndex-len(alphabet)
		if (alphaIndex < len(alphabet)-shiftAmount) :
			alphaIndex += shiftAmount
		
		output += alphabet[alphaIndex]
		
	return output
		
# challenge 0	
print ("Challenge 0 -- 2^38 = "+str(2**38))	

#challenge 1
print ("Challenge 1")
garbledText = "g fmnc wms bgblr rpylqjyrc gr zw fylb. rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq pcamkkclbcb. lmu ynnjw ml rfc spj."
urlCode = "map"
description=transform(garbledText,2)
urlCode=transform(urlCode,2)

print (description)
print ("Solved url code(1): "+urlCode)

#challenge 2
print ("Challenge 2")
file = open('mess-2.txt', 'r')
dataSet = file.readlines()
text = ""

for data in dataSet :
	for char in data: 
		if (char.isalnum()): text+=char
file.close

print (text)
		
#challenge 3
print ("Challenge 3")
output = ""
file = open('mess-3.txt','r')
dataSet = file.readlines()
extracted = "";
for data in dataSet :
	if (len(data)>0): 
		result = re.search('[a-z][A-Z]{3}([a-z])[A-Z]{3}[a-z]',data)
		if (result is not None and len(result.group(0))>0) : extracted += result.group(1)

print (extracted)
file.close

#challenge 4
print ("Challenge 4")
#urllib may help. DON'T TRY ALL NOTHINGS, since it will never end. 400 times is more than enough.
#and the next nothing is 44827
url="http://www.pythonchallenge.com/pc/def/linkedlist.php?nothing="
nothing="12345"

def traverseNothing(url,nothing) :
	result = ""
	while result is not None :
		filehandle = urllib.urlopen(url+nothing)
		fileString = "";
		for text in filehandle: fileString += text #filehandle to string
	
		result = re.search('[0-9].*',fileString) #pull out our "nothing" number
		if result is None: 
			print ("Finished at "+nothing)
			break
		nothing = result.group(0) #set our nothing to the value from the regex
	return nothing
	
#nothing = traverseNothing(url,nothing)	
#print "RESULT NOTHING: "+nothing
#print "Divide by two and keep going? Sure thing! "+nothing+"/2 == "
#nothing = str(int(nothing)/2)
#print "New nothing == "+nothing
#nothing = traverseNothing(url,nothing)
#print ("Nothing: "+nothing)
#nothing = traverseNothing(url,"63579")


