//
//  ContentView.swift
//  SwapiClientIOS
//
//  Created by Igor Steblii on 2/5/20.
//  Copyright © 2020 Igor Steblii. All rights reserved.
//

import SwiftUI

struct ContentView: View {

  @EnvironmentObject var peopleViewModel: PeopleViewModel

  var body: some View {
    NavigationView {
      List(peopleViewModel.items) { item in
        PeopleRow(item: item)
      }
      .navigationBarTitle("SWAPI Rust")
    }
  }

}

struct ContentView_Previews: PreviewProvider {
  static var previews: some View {
    ContentView()
  }
}
