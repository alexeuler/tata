import 'package:flutter/material.dart';

final lightTheme = ThemeData(
  primaryColor: Color(0xFF15B4F1),
  backgroundColor: Color(0xFFFFF9F9),
  textTheme: TextTheme(
    headline1: TextStyle(fontSize: 72.0, fontWeight: FontWeight.bold),
    headline2: TextStyle(fontSize: 48.0, fontWeight: FontWeight.bold),
    bodyText1: TextStyle(fontSize: 16.0),
  ),
);

final darkTheme = lightTheme.copyWith(backgroundColor: Color(0xFF09090F));
