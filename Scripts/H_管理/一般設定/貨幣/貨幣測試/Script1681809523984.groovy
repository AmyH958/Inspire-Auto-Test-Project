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

WebUI.setText(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/input_(15trunk)_Submit'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/a_'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/a__1'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/a__1_2'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/a__1_2_3'))

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/select_AAA - MandyACC - AUD - BL - btc - CH_95af9f'), 
    '9', true)

WebUI.click(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/div_Portal    ERM FRBRFRBRFRBR--()()-- QR C_62097d'))

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/select_AAA - MandyACC - AUD - BL - btc - CH_95af9f_1'), 
    '10', true)

WebUI.click(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/td_AAA - MandyACC - AUD - BL - btc - CHF - _ec58ff'))

WebUI.setText(findTestObject('管理/一般設定/貨幣匯率/新增貨幣匯率/input_INR -_TextField_6'), '20')

WebUI.click(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/a__1_2_3_4'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/a__1_2_3_4_5'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/貨幣匯率/新增貨幣匯率/a__1_2_3_4_5_6'))

WebUI.closeBrowser()

