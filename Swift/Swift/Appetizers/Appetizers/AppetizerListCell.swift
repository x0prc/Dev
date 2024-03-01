//
//  AppetizerListCell.swift
//  Appetizers
//
//  Created by prc on 20/12/23.
//

import SwiftUI

struct AppetizerListCell: View {

    let appetizer : Appetizer
    var body: some View {
        HStack {
            Image("Samosa")
                .resizable()
                .aspectRatio(contentMode: .fit)
//                    .frame(width:)
                .cornerRadius(8)
            
            
            VStack(alignment: .leading, spacing: 5) {
                Text(appetizer.name)
                    .font(.title2)
                    .fontWeight(.medium)
                
                Text("$\(appetizer.price, specifier: "%.2f")")
                    .foregroundColor(.secondary)
                    .fontWeight(.semibold)
                }
            .padding(.leading)
        }
    }
}

struct AppetizerListCell_Previews: PreviewProvider {
    static var previews: some View {
        AppetizerListCell(appetizer: MockData.sampleAppetizer)
    }
}
