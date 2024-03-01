//
//  AppetizerListView.swift
//  Appetizers
//
//  Created by prc on 20/12/23.
//

import SwiftUI

struct AppetizerListView: View {
    
    @State private var appetizers: [Appetizer] = []
    
    
    var body: some View {
        List(appetizers) { appetizer in AppetizerListCell(appetizer: appetizer)
            
            }
            .navigationTitle("Appetizers")
            .onAppear {
                getAppetizers()
        }
        
        }
    
    }
func getAppetizers()  {
    NetworkManager.shared.getAppetizers { result in
        DispatchQueue.main.async {
            switch result {
            case .success(let _): break
            case .failure(let error):
                print(error.localizedDescription)
            }
        }
    }
    
    
    struct AppetizerListView_Previews: PreviewProvider {
        static var previews: some View {
            AppetizerListView()
        }
    }
}
