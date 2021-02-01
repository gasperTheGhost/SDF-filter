// Export specified field from sdf files into new text file for analysis.
// Usage sdreport -f '/path/to/input/file.sdf' (or -d '/path/to/input/dir' and -t sdf,sd,txt) -o /path/to/output/dir -p '<PATTERN>'

import 'dart:io';
import 'package:args/args.dart';

List getFiles(Map input, List fileTypes) {
  if (input['type'] == 'file') {
    //Input is file
    var inputFile = File(input['path']);
    var output = [inputFile];
    return output;
  } else if (input['type'] == 'dir') {
    // input is Directory
    var inputDir = Directory(input['path']);
    var inputArray;
    if (input['recursive']) {
      inputArray = inputDir.listSync(recursive: true, followLinks: true);
    } else {
      inputArray = inputDir.listSync(recursive: false, followLinks: false);
    }
    var output = [];
    // Filter out unwanted and hidden files in directory
    for (var item in inputArray) {
      for (var fileType in fileTypes) {
        if (item.toString().contains('.' + fileType) &&
            !item.toString().contains('/.')) {
          output.add(item);
        }
      }
    }
    return output;
  } else {
    return [];
  }
}

String parseBlock(String SDFblock, String pattern) {
  if (SDFblock.split(pattern).length < 2) {
    print('Error parsing block:');
    print(SDFblock);
    print('Skipping');
    return 'Error parsing block';
  } else {
    var val = SDFblock.split(pattern)[1].split('>')[0].trim();
    if (val == null || val == '') {
      print('Error: no' + pattern + 'value in block:');
      print(SDFblock);
      return 'N/A';
    } else {
      return val;
    }
  }
}

void writeToOutput(String blockToWrite, String fileName) {
  var outputFile = File(fileName);
  if (outputFile.existsSync()) {
    outputFile.writeAsStringSync(blockToWrite, mode: FileMode.append);
  } else {
    outputFile.createSync(recursive: true);
    outputFile.writeAsStringSync(blockToWrite.trimLeft(),
        mode: FileMode.append);
  }
}

void printHelp() {
  print('''
sdreport v0.1 beta
Usage: sdreport -[f/d] /path/to/input(.sdf) -o /path/to/output (-r) -t file,typ -p <PATTERN>
Options:
    -f    --input-file          Absolute path must be specified.
    -d    --input-dir           Absolute path must be specified.
                                Only file OR dir can be used, both
                                will result in error.
    -r    --recursive           Only used with input-dir, enables
                                recursive directory crawling.
                                Off by default

    -o    --output              Output directory (must be absolute).
                                Output file names are the same as
                                originals, but have the suffix '_export.txt'

    -t    --file-types          Comma separated file types to be used
                                Ex. --file-types sdf,sd,txt
                                Files MUST have an extension
                                Defaults to sdf

    -p    --pattern             SDF field to export. Ex: <SCORE>
                                Must end with > character
                                Defaults to <SCORE>''');
  exit(0);
}

void main(List<String> args) {
  var parser = ArgParser();
  parser.addOption('input-file', abbr: 'f');
  parser.addOption('input-dir', abbr: 'd');
  parser.addOption('output', abbr: 'o');
  parser.addOption('pattern', abbr: 'p', defaultsTo: '<SCORE>');
  parser.addMultiOption('file-types', abbr: 't', defaultsTo: ['sdf']);
  parser.addFlag('recursive', abbr: 'r', defaultsTo: false);
  parser.addFlag('help', abbr: 'h');
  var arguments = parser.parse(args);

  // Only one of the inputs may be specified
  var input;
  if (arguments['input-file'] == null && arguments['input-dir'] == null) {
    print('No input specified!');
    printHelp();
  } else if (arguments['input-file'] != null &&
      arguments['input-dir'] != null) {
    print('Only one type of input may be specified!');
    printHelp();
  } else if (arguments['input-dir'] != null) {
    input = {
      'type': 'dir',
      'path': arguments['input-dir'],
      'recursive': arguments['recursive']
    };
  } else if (arguments['input-file'] != null) {
    input = {'type': 'file', 'path': arguments['input-file']};
  }

  var pattern = arguments['pattern'];

  // Get filetypes
  var fileTypes = arguments['file-types'];

  // Help switch handler
  if (arguments['help'] || arguments['output'] == null) {
    printHelp();
  }

  for (var fileName in getFiles(input, fileTypes)) {
    try {
      var contents;
      try {
        contents = fileName.readAsStringSync();
      } catch (e) {
        var fileNameString = fileName.toString().split(' ')[1].split("'")[1];
        contents = Process.runSync(
                'iconv', ['-f', 'UTF-8', '-t', 'UTF-8', '-c', fileNameString])
            .stdout;
      }
      var contentArray = contents.split('\$\$\$\$');
      contentArray.removeLast();
      for (var block in contentArray) {
        writeToOutput(
            parseBlock(block, pattern) + '\n',
            arguments['output'] +
                '/' +
                fileName.toString().split("'")[1].split('/').last +
                '_export.txt');
      }
    } catch (e) {
      print('Error opening file: ' + fileName.toString());
      print(e);
      print('Skipping!');
    }
  }
  print('');
  print('DONE!');
}
