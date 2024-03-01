//
//  ContentView.swift
//  Appetizers
//
//  Created by prc on 20/12/23.
//

import SwiftUI

struct AppetizerViewTab: View {
    var body: some View {
        TabView {
            AppetizerListView()
                .tabItem {
                    Image(systemName: "house")
                    Text("Home")
                }
            AccountView()
                .tabItem {
                    Image(systemName: "person")
                    Text("Account")
                }
            OrderView()
                .tabItem {
                    Image(systemName: "bag")
                    Text("Order")
                }
        }
        .accentColor(Color("brandPrimary"))
            
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        AppetizerViewTab()
    }
}
