//
//  ContentView.swift
//  SwapiClientIOS
//
//  Created by Igor Steblii on 2/5/20.
//  Copyright © 2020 Igor Steblii. All rights reserved.
//

import SwiftUI

struct ContentView: View {

  let actionLoadPeople: (() -> Void)
  @EnvironmentObject var peopleViewModel: PeopleViewModel

  var body: some View {
    NavigationView {
      VStack {
        List(peopleViewModel.items) { item in
          PeopleRow(item: item)
        };
        Button(action: {
          self.actionLoadPeople()
        }) {
          Text("Load people")
          .foregroundColor(.purple)
          .font(.title)
          .padding()
          .border(Color.purple, width: 5)
        };
      }
      .navigationBarTitle("SWAPI Rust")
    }
  }

}

struct ContentView_Previews: PreviewProvider {
  static var previews: some View {
    ContentView(actionLoadPeople: {})
  }
}
