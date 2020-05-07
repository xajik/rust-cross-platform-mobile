//
//  People.swift
//  SwapiClientIOS
//
//  Created by Igor Steblii on 5/5/20.
//  Copyright © 2020 Igor Steblii. All rights reserved.
//

import Foundation

struct People: Identifiable, Codable {
  //list identifier
  var id: UUID? = UUID()
  //Data fields
  var name: String?
  var gender: String?
  var mass: String?
}

extension People {

  static func decode(data: Data) throws -> People? {
    do {
      let decoder = JSONDecoder()
      let user = try decoder.decode(People.self, from: data)
      return user
    } catch let error {
      print(error)
      return nil
    }
  }

}

extension PeopleNative {

  func fromNative() -> People {
    var people = People()
    if let name = self.name {
      people.name = String(cString: name)
    }
    if let height = self.gender {
      people.gender = String(cString: height)
    }
    if let mass = self.mass {
      people.mass = String(cString: mass)
    }
    return people
  }

}
