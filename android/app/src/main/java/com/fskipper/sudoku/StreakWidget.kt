package com.fskipper.sudoku

import android.appwidget.AppWidgetManager
import android.appwidget.AppWidgetProvider
import android.content.Context
import android.content.SharedPreferences
import android.util.Log
import android.widget.RemoteViews

import es.antonborri.home_widget.HomeWidgetProvider

/**
 * Implementation of App Widget functionality.
 */
class StreakWidget : HomeWidgetProvider() {
    override fun onUpdate(
        context: Context,
        appWidgetManager: AppWidgetManager,
        appWidgetIds: IntArray,
        widgetData: SharedPreferences
    ) {

        val streak = widgetData.getInt("acc_streak",0)
        val multiplier = widgetData.getString("acc_multiplier_str","1.0")!!
        val lastStreakDate = widgetData.getLong("acc_last_solved_ms", 0L)
        val now = System.currentTimeMillis()

        val hasDoneToday = lastStreakDate > 0L &&
                java.time.Instant.ofEpochMilli(lastStreakDate)
                    .atZone(java.time.ZoneId.of("Europe/Berlin"))
                    .toLocalDate() ==
                java.time.Instant.ofEpochMilli(now)
                    .atZone(java.time.ZoneId.of("Europe/Berlin"))
                    .toLocalDate()

        // There may be multiple widgets active, so update all of them
        for (appWidgetId in appWidgetIds) {
            updateAppWidget(context, appWidgetManager, appWidgetId,streak,multiplier,hasDoneToday)
        }
    }

    override fun onEnabled(context: Context) {
        // Enter relevant functionality for when the first widget is created
    }

    override fun onDisabled(context: Context) {
        // Enter relevant functionality for when the last widget is disabled
    }
}

internal fun updateAppWidget(
    context: Context,
    appWidgetManager: AppWidgetManager,
    appWidgetId: Int,
    streak: Int,
    multiplier: String,
    hasDoneToday: Boolean
) {


    val streakText = if (streak == 1) "1 Day" else "$streak Days"
    val emojiSource = if (hasDoneToday) R.drawable.fire_color else R.drawable.fire_gray


    // Construct the RemoteViews object
    val views = RemoteViews(context.packageName, R.layout.streak_widget)
    views.setTextViewText(R.id.streak_counter, streakText)
    views.setTextViewText(R.id.multiplierText, "${multiplier}x")
    views.setImageViewResource(R.id.streakEmoji,emojiSource)

    // Instruct the widget manager to update the widget
    appWidgetManager.updateAppWidget(appWidgetId, views)
}