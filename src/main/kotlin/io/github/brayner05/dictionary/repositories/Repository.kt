package io.github.brayner05.dictionary.repositories

import java.util.UUID

interface Repository <V> {
    fun getAll(): Array<V>
    fun getById(id: UUID): V?
    fun add(item: V)
    fun updateById(id: UUID, item: V)
    fun deleteById(id: UUID)
}