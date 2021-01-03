import 'package:flutter/material.dart';
import 'package:tata_mobile/screens/onboarding/onboarding.dart';

void main() {
  runApp(TataApp());
}

final lightTheme = ThemeData(
  primaryColor: Color.fromARGB(0xff, 0x15, 0xB4, 0xF1),
  backgroundColor: Color.fromARGB(0xff, 0xff, 0xf9, 0xf9),
  textTheme: TextTheme(
    headline1: TextStyle(fontSize: 72.0, fontWeight: FontWeight.bold),
    headline2: TextStyle(fontSize: 48.0, fontWeight: FontWeight.bold),
    bodyText1: TextStyle(fontSize: 16.0),
  ),
);

final darkTheme = ThemeData(
  primaryColor: Color.fromARGB(0xff, 0x15, 0xB4, 0xF1),
  backgroundColor: Color.fromARGB(0xff, 0x09, 0x09, 0x0f),
  textTheme: TextTheme(
    headline1: TextStyle(fontSize: 72.0, fontWeight: FontWeight.bold),
    headline2: TextStyle(fontSize: 48.0, fontWeight: FontWeight.bold),
    bodyText1: TextStyle(fontSize: 16.0),
  ),
);

class TataApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Tata',
      theme: darkTheme,
      home: Onboarding(),
    );
  }
}
