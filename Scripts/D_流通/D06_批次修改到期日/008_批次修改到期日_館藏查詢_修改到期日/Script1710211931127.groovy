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

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/input_(15trunk)_member_id'))

WebUI.setText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/a__1'), '批次修改到期日')

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/b_'), '館藏查詢')

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/img__Any_3_0'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/a__1_2'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/td_0000000003'))

WebUI.setText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/input_-_TextField_1'), '0000000003')

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/td_0000000014'))

WebUI.setText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/input_-_TextField_2'), '0000000014')

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/a__1_2'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/input__selectAllRecord'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/img__datePickerImg'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/button_12 , 2024'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/div_'), '確定要修改以下查詢結果嗎？')

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/span_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/div__1'), '修改成功')

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/span_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/p_'), '修改後')

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/p__1'), '結束日期')

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/nobr_20240312'), '2024/03/12')

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/nobr_20240312'), '2024/03/12')

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/a__1_2_3_4'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/div_20240312'), '請確認要修改交易到期日為2024/03/12？')

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/span_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/div__1'), '修改成功')

WebUI.verifyElementText(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/span_'), '確定')

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/span_'))

WebUI.takeFullPageScreenshot()

WebUI.enhancedClick(findTestObject('Object Repository/流通/批次修改到期日/修改到期日發送通知/Page_(15trunk)/a__1_2_3_4_5'))

WebUI.closeBrowser()

