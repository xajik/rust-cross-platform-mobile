package com.rust.app.swapiclient.adapter

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import com.rust.app.swapiclient.R
import com.rust.app.swapiclient.swapi.People

class PeopleAdapter : RecyclerView.Adapter<PeopleAdapter.PeopleViewHolder>() {

    private var data: Array<People>? = null

    override fun onCreateViewHolder(
        parent: ViewGroup,
        viewType: Int
    ): PeopleAdapter.PeopleViewHolder {
        val view = LayoutInflater.from(parent.context).inflate(R.layout.item_people, parent, false)
        return PeopleViewHolder(view)
    }

    override fun onBindViewHolder(holder: PeopleViewHolder, position: Int) {
        data?.let {
            holder.bindView(it[position])
        }
    }

    // Return the size of your dataset (invoked by the layout manager)
    override fun getItemCount() = data?.size ?: 0

    fun setData(data: Array<People>?) {
        this.data = data
    }

    class PeopleViewHolder(view: View) : RecyclerView.ViewHolder(view) {

        val nameView = view.findViewById<TextView>(R.id.peopel_name)
        val massView = view.findViewById<TextView>(R.id.peopel_mass)
        val genderView = view.findViewById<TextView>(R.id.peopel_gender)

        fun bindView(people: People) {
            nameView.text = people.name
            massView.text = people.mass
            genderView.text = people.gender
        }

    }

}