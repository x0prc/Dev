//
//  optionals.swift
//  
//
//  Created by prc on 13/12/23.
//

import UIKit

//optionals (4 methods)
var ages: [Int] = []
ages.sort()

//if let
//if let oldestAge = ages.last {
//    print("oldest age is" \(oldestAge)")
//          } else {
//        print("no oldest age")
//    }
//nil coalescing
let oldestAge = ages.last ?? 999

// guard statement
func getOldestAge(){
    guard let oldestAge = ages.last else {
        return
    }
        print("\(oldestAge) is the oldest age")
    
}

getOldestAge()

//force unwrap
let oldestAge = ages.last!
