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

WebUI.setText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a_'), '管理')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a_'))

WebUI.rightClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a__1'), '一般設定')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a__1_2'), '條碼號規範')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/td_30'), '30')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a_PrefixSufixNumberInventory class'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/img__Any_4'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/b_Prefix'), '[Prefix]')

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/b_Number'), '[Number]')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/img_Sufix_Any_6'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/img_Sufix_Any_6'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/b_Location'), '[Location]')

WebUI.rightClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/img_Number_Any_5_0'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/img_Number_Any_5_0'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/img_Number_Any_4_0'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/img_Number_Any_5_0'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/b_Inventory class'), '[Inventory class]')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/img_Location_Any_4_1'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/td'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/td_'), '前置碼:')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/td'))

WebUI.setText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/input__TextField_0'), '1')

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/td__1'), '後置碼:')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/td__1'))

WebUI.setText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/input__TextField_1'), '6')

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/td__1_2'), '碼:')

WebUI.setText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/input__TextField_2'), '1234567890abc')

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a__1_2_3'), '修改/存檔')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a__1_2_3'))

WebUI.verifyElementPresent(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/div_An exception has occurred              _9a800d'), 
    0, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/span_An exception has occurred'), 'An exception has occurred!', 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a__1_2_3_4'), '請重新登入系統!', FailureHandling.CONTINUE_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/div_Close'), 'Close', FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/div_Close'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a__1_2_3_4_5'), '取消', FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a__1_2_3_4_5'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/修改一筆條碼號規範/a__1_2_3_4_5_6'))

WebUI.closeBrowser()

