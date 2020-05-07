package com.rust.app.swapiclient.nativeswapi

import android.util.Log
import kotlinx.coroutines.*
import kotlinx.serialization.Serializable
import kotlinx.serialization.UnstableDefault
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonConfiguration
import java.net.URL
import kotlin.coroutines.CoroutineContext

class NativeSwapiClient : CoroutineScope {

    companion object {
        const val TAG = "NativeSwapiClient"
        const val SWAPI_PEOPLE = "https://swapi.dev/api/people/"
    }

    private var job: Job = Job()

    override val coroutineContext: CoroutineContext
        get() = Dispatchers.Main + job

    @UnstableDefault
    fun loadPeople() {
        val before = System.currentTimeMillis()
        launch {
            val results = withContext(Dispatchers.IO) { load() }
            val after = System.currentTimeMillis()
            val diff = after - before
            Log.e(TAG, "For ${results.size}; elapsed Time: $diff")
        }
    }

    @UnstableDefault
    private fun load(): List<People> {
        val text = URL(SWAPI_PEOPLE).readText()
        val json = Json(JsonConfiguration(strictMode = false))
        val data: PeopleWrapper = json.parse(PeopleWrapper.serializer(), text)
        return data.results ?: emptyList()
    }

}

@Serializable
private data class PeopleWrapper(
    val count: Int?,
    val results: List<People>?
)

@Serializable
private data class People
    (
    val name: String,
    val mass: String,
    val gender: String
)