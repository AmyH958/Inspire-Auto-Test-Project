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

WebUI.maximizeWindow()

WebUI.enableSmartWait()

WebUI.setText(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/input_(15trunk)_Submit'))

WebUI.enableSmartWait()

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/td_30'), '30')

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/a_PrefixSufixNumberInventory class'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/a__1_2_3'), '刪除')

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/div_'), '您確定要刪除?')

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/span_'), '刪除')

WebUI.takeFullPageScreenshotAsCheckpoint('刪除視窗')

WebUI.enableSmartWait()

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/span_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/td_29'), '29')

WebUI.takeFullPageScreenshotAsCheckpoint('刪除成功')

WebUI.enableSmartWait()

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/刪除一筆條碼號規範/a__1_2_3_4'))

WebUI.closeBrowser()

