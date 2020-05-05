//
//  SwapiLoader.swift
//  SwapiClientIOS
//
//  Created by Igor Steblii on 4/5/20.
//  Copyright © 2020 Igor Steblii. All rights reserved.
//

import Foundation


//Wrapper for Rust SwapiClient
class SwapiLoader {

  private let client: OpaquePointer

  init() {
    client = create_swapi_client()
  }

  deinit {
    free_swapi_client(client)
  }

  func loadPeople(resultsCallback: @escaping (([People]) -> Void), errorCallback: @escaping (String) -> Void) {

    //We cannot call callback from C context, we need to send reference to callback to C
    let callbackWrapper = PeopleResponse(onSuccess: resultsCallback, onError: errorCallback)

    //pointer to callback class
    let owner = UnsafeMutableRawPointer(Unmanaged.passRetained(callbackWrapper).toOpaque())

    //C callback results
    var onResult: @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<PeopleNativeWrapper>?) -> Void = {
      let owner: PeopleResponse = Unmanaged.fromOpaque($0!).takeRetainedValue()
      if let data:PeopleNativeWrapper = $1?.pointee {
        print("data \(data.length)")
        let buffer = data.asBufferPointer
        var people = [People]()
        for b in buffer {
          people.append(b.fromNative())
        }
        owner.onSuccess(people)
      }
    }

    //C callback error
    var onError: @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<Int8>?) -> Void = {
      guard let pointer = $1 else {return;}
      let owner: PeopleResponse = Unmanaged.fromOpaque($0!).takeRetainedValue()
      let error = String(cString: pointer)
      owner.onError(error)
    }

    //Callback struct defined in Rust
    var callback = PeopleCallback (
      owner: owner,
      onResult: onResult,
      onError: onError
    )

    load_all_people(client, callback)
  }

}

//Helper to change context from Rust to Swift
class PeopleResponse {
  public let onSuccess: (([People]) -> Void)
  public let onError: ((String) -> Void)
  init(onSuccess: @escaping (([People]) -> Void), onError: @escaping ((String) -> Void)) {
    self.onSuccess = onSuccess
    self.onError = onError
  }
}

//Transform C array [pointe; lenght] to Swift array
extension PeopleNativeWrapper {
  var asBufferPointer: UnsafeMutableBufferPointer<PeopleNative> {
    return UnsafeMutableBufferPointer(start: array, count: Int(length))
  }
}
