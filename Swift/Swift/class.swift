//
//  class.swift
//  
//
//  Created by prc on 13/12/23.
//

import UIKit

class Developer {
    var name: String
    var jobTitle: String
    var yearsExp: Int
    
    init(name: String, jobTitle: String, yearsExp: Int){
        self.name = name
        self.jobTitle = jobTitle
        self.yearsExp = yearsExp
    }
}

let prc = Developer()
//creates optionals
prc.name
prc.jobTitle
prc.yearsExp
