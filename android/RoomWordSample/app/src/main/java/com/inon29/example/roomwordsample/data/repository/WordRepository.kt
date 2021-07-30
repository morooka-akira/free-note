package com.inon29.example.roomwordsample.data.repository

import androidx.annotation.WorkerThread
import com.inon29.example.roomwordsample.data.db.dao.WordDao
import com.inon29.example.roomwordsample.data.db.entity.Word
import kotlinx.coroutines.flow.Flow

class WordRepository(private val wordDao: WordDao) {

    val allWords: Flow<List<Word>> = wordDao.getAlphabetizedWords()

    @Suppress("RedundantSuspendModifier")
    @WorkerThread
    suspend fun insert(word: Word) {
        wordDao.insert(word)
    }
}