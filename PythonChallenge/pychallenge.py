#An attempt to learn python while completing the challenges specified at http://www.pythonchallenge.com/

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
		
#challenge 3		
