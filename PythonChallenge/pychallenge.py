#An attempt to learn python while completing the challenges specified at http://www.pythonchallenge.com/
import re
import urllib
import pickle 

alphabet = "abcdefghijklmnopqrstuvwxyz"

def transform(inputString,shiftAmount):
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
		
		
print(2**38) # challenge 0

#challenge 1
garbledText = "g fmnc wms bgblr rpylqjyrc gr zw fylb. rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq pcamkkclbcb. lmu ynnjw ml rfc spj."
urlCode = "map"
shiftVal = 2
print transform(garbledText,shiftVal)
print "Solved url code(1): "+transform(urlCode,shiftVal)

#challenge 2
file = open('mess-2.txt', 'r')
dataSet = file.readlines()
for data in dataSet :
	for char in data: 
		if (char.isalnum()): print char
file.close
		
#challenge 3
output = ""
file = open('mess-3.txt','r')
dataSet = file.readlines()
extracted = "";
for data in dataSet :
	if (len(data)>0): 
		result = re.search('[a-z][A-Z]{3}([a-z])[A-Z]{3}[a-z]',data)
		if (result is not None and len(result.group(0))>0) : extracted += result.group(1)

print extracted
file.close

#challenge 4
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
			print ("Finished at"+nothing)
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

#challenge 5 - pickle http://www.pythonchallenge.com/pc/def/peak.html
results = pickle.load( open( "banner.p", "rb" ) )
#for text in results :
	#print text
