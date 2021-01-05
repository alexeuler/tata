import 'package:flutter/material.dart';
import 'package:tata_mobile/screens/onboarding/onboarding.dart';
import 'package:tata_mobile/themes.dart';
import 'package:theme_manager/theme_manager.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(TataApp());
}

class TataApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ThemeManager(
      defaultBrightnessPreference: BrightnessPreference.light,
      data: (Brightness brightness) {
        switch (brightness) {
          case Brightness.light:
            return lightTheme;
          case Brightness.dark:
            return darkTheme;
        }
        return lightTheme;
      },
      loadBrightnessOnStart: true,
      themedWidgetBuilder: (BuildContext context, ThemeData theme) {
        return MaterialApp(
          title: 'Tata',
          theme: theme,
          home: Onboarding(),
        );
      },
    );
  }
}
