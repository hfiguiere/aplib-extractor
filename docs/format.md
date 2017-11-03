Aperture Library format



Table 1:

App version | DB version | DB minor | Project vers
------------+------------+----------+-------------
3.1.3       | 110        | 122      | 6
3.2.2       | 110        | 131      | 7
3.2.4       | 110        | 131      | 7
3.3.2       | 110        | 207 (203)| 8
3.4.5       | 110        | 219 (208)| 8
3.6         | 110        | 226 (220)| 8


Bundle structure
================

This is the file structure of the bundle for version 110. Between [ ]
is the earliest DB minor we *saw* the file.


Aperture.aplibrary
|
+- Aperture.aplib
|  +- DataModelVersion.plist
|
+- ApertureData.xml [131]
+- Attachments
+- Database
|  +- ActiveWebPublishingAccounts.plist [207]
|  +- Albums
|  |  +- *.apalbum
|  |  +-
|  |
|  +- apdb
|  |  +- BigBlobs.apdb
|  |  +- Faces.db
|  |
|  +- BigBlobs.apdb -> apdb/BigBlobs.apdb
|  +- DataModelVersion.plist
|  +- Faces
|  |  +- Detected
|  |     +- *.apdetected
|  |  +- DetectedExternals
|  |  +- FaceExternals
|  |  +- FaceNames
|  |
|  +- Faces.db -> apdb/Faces.db
|  +- Folders
|  |  +- *.apfolder
|  |
|  +- History
|  |  +- Changes
|  |     +- *.plist
|  |
|  +- History.apdb -> apdb/History.apdb
|  +- ImageProxies.apdb -> apdb/ImageProxies.apdb
|  +- KeywordSets.plist [207]
|  +- Keywords.plist
|  +- Library.apdb -> apdb/Library.apdb
|  +- Places
|  |  +- *.applace
|  |
|  +- Properties.apdb -> apdb/Properties.apdb
|  +- tmSync.plist
|  +- Vaults
|  +- Versions
|  |  +- YYYY
|  |     +- MM
|  |        +- DD
|  |           +- YYYYMMDD-nnnnnn
|  |              +- <id>
|  |                 +- Master.apmaster
|  |                 +- Version-0.apversion
|  |                 +- Version-1.apversion
|  |
|  +- Volumes
|     +- *.apvolume
|  +- tmSync.plist
|
+- iLifeShared
|  +- ApertureDatabaseTimestamp
+- iMovie-Thumbnails [optional?]
+- iPod Photo Cache [optional?]
|
+- Info.plist
+- Masks
+- Masters
|  +- YYYY
|     +- MM
|        +- DD
|           +- YYYYMMDD-nnnnnn
|             +- *
|
+- Previews
+- Thumbnails



ApertureData.xml: seems to contain a dump of the whole data model but
it seems to not be present everywhere. [IGNORE]

Aperture.aplib
==============

This seems to have only one file.

DataModelVersion.plist
----------------------

It specifies the datamodel, the version for projects and a few other
details

Properties:

* DatabaseCompatibleBackToMinorVersion: DB minor version back compatible. (-226)
* DatabaseMinorVersion (integer): DB minor version. See Table 1. (-226)
* DatabaseVersion (integer): DB version. See Table 1. (-226)
* adminProperties (dict): various properties / settings. (226)
* createDate (date): creation date (-226)
* databaseUuid (string): UUID (-226)
* imageIOVersion (string): ? (!226)
* isIPhotoLibrary (bool): false (-226)
* masterCount (integer): count of masters (-226)
* migratedMobileMeAccounts (array): ? (!226)
* projectVersion (integer): project version. See Table 1.
* projectCompatibleBackToVersion (integer): See 'projectVersion'
* rawCameraBundleVersion (string): ? (!226)
* touchedByAperture (bool): true ? (226)
* versionCount (integer): count of versions (-226)


Database
========

Database/Folders is all the containers, folders, project, etc.
The .apfolder files are binary plists.

Database/Albums contain the albums from the library. The .apalbum
files are binary plists.

Common plist properties
-----------------------

* uuid: a string uuid.
* modelId: numeral id. Possibly unique library wide.
* folderUuid: uuid of the folder this is contained in
* parentFolderUuid: for Folders, the parent.

Notes
-----

Notes are under a `notes` plist property. Found in folders and masters. It is 
an array that contains dictionaries.

Common properties:

* attachedToUuid: uuid it is attached too (uuid in the containing plist)
* createDate
* uuid: uuid of the note.

### Folders

* note: text note. For Aperture project it is in the project info

### Masters

Master has `hasFocusPoints` set to true. Attached in the notes:

* propertyKey: focusPoints
* data
* modelId.

Folders
-------

All in top level.

* implicitAlbumUuid: the uuid of the album that is representing the view
  (Subclass 2 album)
* posterVersionUuid: the uuid of the version that is the poster for the
  Project (type = 2 [Project])
* notes: an array of dict (multiple Notes)

Albums
------

Unlike other plist definitions, albums have two levels.

Top-level properties:

* UserQueryInfo: the query for smart album. DATA.
* InfoDictionary: these are the actual properties
* attachments: attachments like track path
* FilterInfo: display filter. DATA.
* versionUuids: An array of uuid: the versions it contains. (Subclass 3)

### InfoDictionary

This is the main set of properties.

* selectedTrackPathUuid: the UUID of the track selected. See attachments.

* albumSubclass:
Subclass 1 Albums are attached to a folder. Linked via the folder
`implicitAlbumUuid` property and back with albums `folderUuid`. They
represent the view of the folder.
Sublclass 2 Albums are "smart", they are backed by a query.
Sublclass 3 Albums are "user", ie created by the user to contain versions.
See the "versionUuids" array for the list of albums it contains.


Keywords.plist
--------------

Define the keywords in the database. A plist with hierarchial keywords.

Properties:
* keywords_verions (integer): 6 or 7. Not sure which is what, I don't
see difference otherwise.


Versions
--------

The stem of the filename is probably irrelevant.

Common properties:

* createDate: date of when it was created.

### Master.apmaster ###

Description of the master. Each version has a master.

* type: IMGT is image.
* subtype: RAWST is RAW. JPGST is JPEG. TIFST is TIFF.
* importGroupUuid: uuid for the import group. - apparently no other info.
* alternateMasterUuid: the other master (for JPEG+RAW) - reciprocal
* originalVersionUuid: the uuid of the original version. Likely n=0.
* modelId: numerical ID
* fileVolumeUuid: the UUID of the volume. See Volumes
* fileIsReference: true if not physically in library (referenced file in UI)
* projectUuid: the uuid of the project it is in (see Folders)
* pixelFormat: (int). 6 for a CR2.
* hasFocusPoints: If set to true the data is found in the `notes` property.
* colorSpaceDefinition: Found with TIFF masters.
* faceDetectionState: int. Values found: 9.

### Version-n.apversion ###

* isFlagged: version flagged
* isOriginal: this is the original version. Usually n=0.
* isEditable
* isHidden
* isInTrash
* imageTimeZoneName: timezone name for the image dates.
* exportImageChangeDate: (date) when it was last exported.
* exportMetadataChangeDate: (date) when metadata was last changed
   (not on version 0)
* rawMasterUuid: uuid of RAW master
* nonRawMasterUuid: uuid of non-RAW master.
* showInLibrary: whether to show. false likely to be implicit version of
  master.
* name: version name
* fileName: filename for version
* mainRating: rating
* rotation: Image rotation in degrees.
* versionNumber: the version number. n in the filename.
* iptcProperties: IPTC
* exifProperties: EXIF
* renderVersion: ???? (is this related to the RAW decoder version
   from adjustmentProperties.RawDecodeVersion)
* customInfo: struct containing timezone of the camera and picture's.

* hasAdjustments: bool. Always true.
* hasEnabledAdjustments: bool. If any adjustement past RAW decode
   is applied.
* RKImageAdjustments: array of dict for adjustement. Always one item
   for RAW decode.

Volumes
-------

### <uuid>.apvolume ###

A volume where to find files.

* diskUuid: the disk UUID (OS?)
* modelId: numerical model ID
* uuid: the object UUID. Referenced from fileVolumeUuid in master
* volumeName: OS volume name.

Masters
=======


