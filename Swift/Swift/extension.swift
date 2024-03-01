//
//  extension.swift
//  
//
//  Created by prc on 13/12/23.
//

import Foundation
extension String {
    func removeWhitespace() -> String {
        return components(separatedBy: .whitespaces).joined()
        
    }
}

let alphabet = "A B C D E F"
print(alphabet.removeWhitespace())
