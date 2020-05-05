//
//  PeopleViewModel.swift
//  SwapiClientIOS
//
//  Created by Igor Steblii on 5/5/20.
//  Copyright © 2020 Igor Steblii. All rights reserved.
//

import Foundation


class PeopleViewModel: ObservableObject {

  @Published var items: [People] = []

  public func setData(newItmes: [People]) {
    items.removeAll()
    items.append(contentsOf: newItmes)
  }

}
