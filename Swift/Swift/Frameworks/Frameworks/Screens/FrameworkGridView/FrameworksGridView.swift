//
//  FrameworksGridView.swift
//  Frameworks
//
//  Created by prc on 18/12/23.
//

import SwiftUI

struct FrameworksGridView: View {
    
    @StateObject var viewModel = FrameworkGridViewModel()
    
    
    
    var body: some View {
        NavigationView {
            List {
                ForEach(MockData.frameworks) { framework in
                    NavigationLink(destination:
                                    FrameworkDetailView(framework: framework,
                                                        isShowingDetailView: $viewModel.isShowingDetailView)){
                                    FrameworkTitleView(framework: framework)
                    }
                    
                }
                   
            }
            
            .navigationTitle("Frameworks")            
            }
        .accentColor(Color(.label))
        }
    }

struct FrameworksGridView_Previews: PreviewProvider {
    static var previews: some View {
        FrameworksGridView()
            .preferredColorScheme(.dark)
    }
}

