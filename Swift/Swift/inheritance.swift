//
//  inheritance.swift
//  
//
//  Created by prc on 13/12/23.
//

import UIKit

class iOSDev: Developer {
    var favoriteFramework: String?
    
    func speakFavoriteFramework() {
        if let favoriteFramework = favoriteFramework {
            print(favoriteFramework)
        } else {
            print("I dont have a favorite framework")
        }
    }
    
    override func speakName() {
        print("\(name!)" - \(jobTitle!)")
    }
}

let prc = iOSDev

prc.favoriteFramework = "ARKit"

