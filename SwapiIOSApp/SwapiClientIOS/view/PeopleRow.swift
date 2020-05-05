//
//  PeopleRow.swift
//  SwapiClientIOS
//
//  Created by Igor Steblii on 5/5/20.
//  Copyright © 2020 Igor Steblii. All rights reserved.
//

import Foundation
import SwiftUI


struct PeopleRow : View {

  var item: People

  var body: some View {
    HStack {
      Text("Name: ").bold();
      Text(item.name ?? "N/A");
      Text("Gender: ").bold();
      Text(item.gender ?? "N/A");
      Text("Mass: ").bold();
      Text(item.mass ?? "N/A");
    }
  }

}
