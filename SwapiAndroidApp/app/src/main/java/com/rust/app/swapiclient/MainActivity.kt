package com.rust.app.swapiclient

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.widget.Toast
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.rust.app.swapiclient.adapter.PeopleAdapter
import com.rust.app.swapiclient.swapi.People
import com.rust.app.swapiclient.swapi.SwapiClient
import com.rust.app.swapiclient.swapi.SwapiPeopleLoadedListener

class MainActivity : AppCompatActivity() {

    companion object {
        const val TAG = "MainActivity"

        init {
            System.loadLibrary("swapi")
        }
    }

    private lateinit var recyclerView: RecyclerView
    private lateinit var viewAdapter: PeopleAdapter
    private lateinit var viewManager: RecyclerView.LayoutManager

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        setupRecycler()
    }

    private fun setupRecycler() {
        viewManager = LinearLayoutManager(this)
        viewAdapter = PeopleAdapter()
        recyclerView = findViewById<RecyclerView>(R.id.recycler_view).apply {
            layoutManager = viewManager
            adapter = viewAdapter
        }
    }

    override fun onStart() {
        super.onStart()
        loadPeople()
    }

    private fun loadPeople() {
        val client = SwapiClient()
        client.loadAllPeople(object : SwapiPeopleLoadedListener {

            override fun onLoaded(s: Array<People>) {
                viewAdapter.setData(s)
                runOnUiThread {
                    viewAdapter.notifyDataSetChanged()
                }
            }

            override fun onError(s: String) {
                Log.e(TAG, s)
                runOnUiThread {
                    Toast.makeText(this@MainActivity, s, Toast.LENGTH_LONG).show()
                }
            }

        })
    }

}