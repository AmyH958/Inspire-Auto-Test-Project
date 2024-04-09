import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.enableSmartWait()

WebUI.maximizeWindow()

WebUI.enableSmartWait()

WebUI.setText(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/input_(15trunk)_Submit'))

WebUI.enableSmartWait()

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a_'))

WebUI.mouseOver(findTestObject('管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a__1'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a__1'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a__1_2'))

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/select_EnglishRomn'), '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/select_EnglishRomn_1'), '5', true)

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/input_2_searchLink'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a_2'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a_3'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a__DirectLink_6'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a__DirectLink_7'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a__DirectLink_7'))

WebUI.takeFullPageScreenshotAsCheckpoint('檢視翻譯清單資料')

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a__1_2_3'))

WebUI.takeFullPageScreenshotAsCheckpoint('檢視語系清單')

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a__1_2_3_4'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/翻譯轉換/檢視翻譯及語系清單/a__1_2_3_4_5'))

WebUI.closeBrowser()

