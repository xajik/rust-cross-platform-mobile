//
//  PeopleResponse.swift
//  SwapiClientIOS
//
//  Created by Igor Steblii on 7/5/20.
//  Copyright © 2020 Igor Steblii. All rights reserved.
//

import Foundation

struct PeopleRespose : Codable {
  var count: Int? = nil
  var next: String? = nil
  var results: [People]? = nil
}


extension PeopleRespose {

  static func decode(data: Data) throws -> PeopleRespose? {
    do {
      let decoder = JSONDecoder()
      let user = try decoder.decode(PeopleRespose.self, from: data)
      return user
    } catch let error {
      print(error)
      return nil
    }
  }

}
