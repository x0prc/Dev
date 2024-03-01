//sets
var ages = [21, 55, 189, 23, 234]
var agesSet: Set<int> = []

ages.count
ages.last
ages[3]

ages.append
ages.insert(44, at:0)
ages.shuffle()
print(ages)

//constant time lookup 
agesSet.contains(23)
print(agesSet)

//dictionaries
let devices: [String: String] = [
	"gamer": "yes",
	"baller": "yes",
	"broke": "nope",
	"based": "nope",
]

devices["gamer"]

//functions
func printInstructors(name: String) {
	print(name)
}
printInstructors(name: "joemama")


func add(firstNumber: Int, to secondNumber: Int) -> Int {
	
	let sum = firstNumber + secondNumber
	return sum
}
add(firstNumber: 44, to: 12)

//if else
var highScore = 111
if highScore > 500 {
    print("you are the best")
} else if highScore > 240 {
    print("you are average")
} else if highScore > 100 {
    print("you suck balls")
} else {
    print("stfu")
}

//loops
let allStars = ["james", "davis", "harden", "doncic", "leonard"]
for player in allStars where player == "Harden"{
    print(player)
}

var randomInts: [Int] = []

//ranges
for i in 0..<25 {
    let randomNumber = Int.random(in: 0...100)
    randomInts.append(randomNumber)
    
}

print(randomInts)

//enum
enum Phone: String {
    case iPhone = "this is my gamer phone"
    case Mac = "baller system"
    case pixel = "nuh uh"
    case Nokia = "def nuh uh"
}
//raw values
func getMyOpinion(on phone: Pho/Users/profic1ous/Documents/GitHub/Swift/optionals.swiftne){
    print(phone.rawValue)
}

//switch statement (can use ranges in this as well)
func getMyOpinion(on phone: Phone){
    switch phone {
    case .iPhone:
        print("ayyyy")
    }
}

getMyOpinion(on: .iPhone)
getMyOpinion(on: .pixel)


















