//
//  SceneDelegate.swift
//  SwapiClientIOS
//
//  Created by Igor Steblii on 2/5/20.
//  Copyright © 2020 Igor Steblii. All rights reserved.
//

import UIKit
import SwiftUI

class SceneDelegate: UIResponder, UIWindowSceneDelegate {

  var window: UIWindow?

  var peopleViewModel = PeopleViewModel()
  //Rust client
  lazy var client = SwapiLoader()
  //SwiftClient
  lazy var nativeClient = NativeSwapiClient()

  func scene(_ scene: UIScene, willConnectTo session: UISceneSession, options connectionOptions: UIScene.ConnectionOptions) {
    // Use this method to optionally configure and attach the UIWindow `window` to the provided UIWindowScene `scene`.
    // If using a storyboard, the `window` property will automatically be initialized and attached to the scene.
    // This delegate does not imply the connecting scene or session are new (see `application:configurationForConnectingSceneSession` instead).

    // Create the SwiftUI view that provides the window contents.
    let contentView = ContentView(actionLoadPeople: {

      self.loadPeople()
      //Compare with native swift execution time
      //self.loadPeopleNative()
    })

    // Use a UIHostingController as window root view controller.
    if let windowScene = scene as? UIWindowScene {
      let window = UIWindow(windowScene: windowScene)
      window.rootViewController = UIHostingController(rootView: contentView.environmentObject(peopleViewModel))
      self.window = window
      window.makeKeyAndVisible()
    }
  }

  func sceneDidDisconnect(_ scene: UIScene) {
    // Called as the scene is being released by the system.
    // This occurs shortly after the scene enters the background, or when its session is discarded.
    // Release any resources associated with this scene that can be re-created the next time the scene connects.
    // The scene may re-connect later, as its session was not neccessarily discarded (see `application:didDiscardSceneSessions` instead).
  }

  func sceneDidBecomeActive(_ scene: UIScene) {
    // Called when the scene has moved from an inactive state to an active state.
    // Use this method to restart any tasks that were paused (or not yet started) when the scene was inactive.
  }

  //Rust client
  func loadPeople() {
    let methodStart = Date()
    client.loadPeople(resultsCallback: {[weak self] data in
      guard let self = self else {return;}

      let methodFinish = Date()
      let executionTime = methodFinish.timeIntervalSince(methodStart)
      print("Rust execution time: \(executionTime)")

      DispatchQueue.main.async {
        self.peopleViewModel.setData(newItmes: data)
      }
      }, errorCallback: {error in
        print("Error: \(error)")
    })
  }

  //Swift client
  func loadPeopleNative() {
    let methodStart = Date()
    nativeClient.loadPeople { [weak self] result in
      guard let self = self else {return;}

      let methodFinish = Date()
      let executionTime = methodFinish.timeIntervalSince(methodStart)
      print("Swift execution time: \(executionTime)")

      if let people = result {
        DispatchQueue.main.async {
          self.peopleViewModel.setData(newItmes: people)
        }
      }
    }
  }

  func sceneWillResignActive(_ scene: UIScene) {
    // Called when the scene will move from an active state to an inactive state.
    // This may occur due to temporary interruptions (ex. an incoming phone call).
  }

  func sceneWillEnterForeground(_ scene: UIScene) {
    // Called as the scene transitions from the background to the foreground.
    // Use this method to undo the changes made on entering the background.
  }

  func sceneDidEnterBackground(_ scene: UIScene) {
    // Called as the scene transitions from the foreground to the background.
    // Use this method to save data, release shared resources, and store enough scene-specific state information
    // to restore the scene back to its current state.
  }

}

