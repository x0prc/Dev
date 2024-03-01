//
//  FrameworkGridViewModel.swift
//  Frameworks
//
//  Created by prc on 18/12/23.
//

import SwiftUI

class FrameworkGridViewModel: ObservableObject {
    
    var selectedFramework : Framework? {
        didSet {isShowingDetailView = true}
    }
    
    @Published var isShowingDetailView = false
    let columns : [GridItem] = [GridItem(.flexible()),
                                GridItem(.flexible()),
                                GridItem(.flexible())]
}
