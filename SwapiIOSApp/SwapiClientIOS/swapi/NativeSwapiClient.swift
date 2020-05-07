//
//  NativeSwapiClient.swift
//  SwapiClientIOS
//
//  Created by Igor Steblii on 7/5/20.
//  Copyright © 2020 Igor Steblii. All rights reserved.
//

import Foundation

private let SwapiURL = "https://swapi.dev/api/people/";

class NativeSwapiClient {

  func loadPeople(onSuccess: @escaping (([People]?) -> Void)) {
    guard let url = URL(string: SwapiURL) else {return}
    let task = URLSession.shared.dataTask(with: url) { (data, response, error) in
      guard let dataResponse = data,
        error == nil else {
          print(error?.localizedDescription ?? "Response Error")
          return }
      if let result = try? PeopleRespose.decode(data: dataResponse) {
        onSuccess(result.results);
      }
    }
    task.resume()
  }

}
